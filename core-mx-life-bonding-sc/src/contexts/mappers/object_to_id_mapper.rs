use core::{borrow::Borrow, marker::PhantomData};

use multiversx_sc::{
    api::{ErrorApiImpl, StorageMapperApi},
    codec::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    storage::{mappers::StorageMapper, StorageKey},
    storage_clear, storage_get, storage_get_len, storage_set,
    types::ManagedType,
};

static ID_SUFFIX: &[u8] = b"id";
static OBJECT_SUFFIX: &[u8] = b"object";
static UNKNOW_OBJECT_ERR_MSG: &[u8] = b"Unknown object";
static LAST_ID_SUFFIX: &[u8] = b"lastId";

pub type Id = u64;
pub const NULL_ID: Id = 0;

pub struct ObjectToIdMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
{
    _phantom_api: PhantomData<SA>,
    _phantom_item: PhantomData<T>,
    base_key: StorageKey<SA>,
}

impl<SA, T> StorageMapper<SA> for ObjectToIdMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode,
{
    #[inline]
    fn new(base_key: StorageKey<SA>) -> Self {
        ObjectToIdMapper {
            _phantom_api: PhantomData,
            _phantom_item: PhantomData,
            base_key,
        }
    }
}

impl<SA, T> ObjectToIdMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode,
{
    pub fn contains_id(&self, id: Id) -> bool {
        let key = self.id_to_object_key(id);
        storage_get_len(key.as_ref()) != 0
    }

    pub fn get_id<BT>(&self, object: BT) -> Id
    where
        BT: Borrow<T>,
    {
        let key = self.object_to_id_key(object);
        storage_get(key.as_ref())
    }

    pub fn get_id_non_zero<BT>(&self, object: BT) -> Id
    where
        BT: Borrow<T>,
    {
        let id = self.get_id(object);
        if id == NULL_ID {
            SA::error_api_impl().signal_error(UNKNOW_OBJECT_ERR_MSG);
        }

        id
    }

    pub fn get_object(&self, id: Id) -> Option<T> {
        let key = self.id_to_object_key(id);
        if storage_get_len(key.as_ref()) == 0 {
            return None;
        }
        let object = storage_get(key.as_ref());
        Some(object)
    }

    fn id_to_object_key(&self, id: Id) -> StorageKey<SA> {
        let mut item_key = self.base_key.clone();
        item_key.append_bytes(ID_SUFFIX);
        item_key.append_item(&id);

        item_key
    }

    fn object_to_id_key<BT>(&self, object: BT) -> StorageKey<SA>
    where
        BT: Borrow<T>,
    {
        let mut item_key = self.base_key.clone();
        item_key.append_bytes(OBJECT_SUFFIX);
        item_key.append_item(object.borrow());

        item_key
    }

    fn last_id_key(&self) -> StorageKey<SA> {
        let mut item_key = self.base_key.clone();
        item_key.append_bytes(LAST_ID_SUFFIX);

        item_key
    }

    pub fn get_last_id(&self) -> Id {
        storage_get(self.last_id_key().as_ref())
    }
}

impl<SA, T> ObjectToIdMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode,
{
    pub fn get_id_or_insert(&self, object: T) -> Id {
        let current_id = storage_get(self.object_to_id_key(&object).as_ref());
        if current_id != 0 {
            return current_id;
        }

        self.insert_object(object)
    }

    pub fn insert_new(&self, object: T) -> Id {
        let existing_id = self.get_id(&object);
        if existing_id != NULL_ID {
            SA::error_api_impl().signal_error(b"Object already registered");
        }

        self.insert_object(object)
    }

    pub fn remove_by_id(&self, id: Id) -> Option<T> {
        let object = self.get_object(id)?;
        self.remove_entry(id, &object);

        Some(object)
    }

    pub fn remove_by_object(&self, object: T) -> Id {
        let current_id = self.get_id(&object);
        if current_id != NULL_ID {
            self.remove_entry(current_id, object);
        }

        current_id
    }

    fn insert_object(&self, object: T) -> Id {
        let new_id = self.get_last_id() + 1; //starts from 1
        storage_set(self.id_to_object_key(new_id).as_ref(), &object);
        storage_set(self.object_to_id_key(object).as_ref(), &new_id);

        self.set_last_id(new_id);

        new_id
    }

    fn set_last_id(&self, last_id: Id) {
        if last_id == 0 {
            SA::error_api_impl().signal_error(b"ID Overflow");
        }

        storage_set(self.last_id_key().as_ref(), &last_id);
    }

    fn remove_entry<BT>(&self, id: Id, object: BT)
    where
        BT: Borrow<T>,
    {
        storage_clear(self.object_to_id_key(object).as_ref());
        storage_clear(self.id_to_object_key(id).as_ref());
    }
}
