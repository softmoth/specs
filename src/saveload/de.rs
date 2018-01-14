use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;

use serde::de::{self, Deserialize, DeserializeOwned, DeserializeSeed, Deserializer, SeqAccess,
                Visitor};

use error::NoError;
use saveload::EntityData;
use saveload::storages::GenericWriteStorage;
use saveload::marker::{Marker, MarkerAllocator};
use storage::WriteStorage;
use world::{Component, EntitiesRes, Entity};

/// A trait which allows to deserialize entities and their components.
pub trait DeserializeComponents<E, M>
where
    Self: Sized,
    E: Display,
    M: Marker,
{
    /// The data representation that a component group gets deserialized to.
    type Data: DeserializeOwned;

    /// Loads `Component`s to entity from `Data` deserializable representation
    fn deserialize_entity<'a, F>(
        &mut self,
        entity: Entity,
        components: Self::Data,
        ids: F,
    ) -> Result<(), E>
    where
        F: FnMut(M) -> Option<Entity>;

    /// Deserialize entities according to markers.
    fn deserialize<'a: 'b, 'b, 'de, D>(
        &'b mut self,
        entities: &'b EntitiesRes,
        markers: &'b mut WriteStorage<'a, M>,
        allocator: &'b mut M::Allocator,
        deserializer: D,
    ) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(VisitEntities::<E, M, Self> {
            allocator,
            entities,
            markers,
            storages: self,
            pd: PhantomData,
        })
    }
}

/// Wrapper for `Entity` and tuple of `WriteStorage`s that implements `serde::Deserialize`.
struct DeserializeEntity<'a: 'b, 'b, 's, E, M: Marker, S: 's> {
    allocator: &'b mut M::Allocator,
    entities: &'b EntitiesRes,
    storages: &'s mut S,
    markers: &'b mut WriteStorage<'a, M>,
    pd: PhantomData<E>,
}

impl<'de, 'a: 'b, 'b, 's, E, M, S> DeserializeSeed<'de> for DeserializeEntity<'a, 'b, 's, E, M, S>
where
    E: Display,
    M: Marker,
    S: DeserializeComponents<E, M> + 's,
{
    type Value = ();
    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        let DeserializeEntity {
            entities,
            storages,
            markers,
            allocator,
            ..
        } = self;
        let data = EntityData::<M, S::Data>::deserialize(deserializer)?;
        let entity = allocator.get_or_create(data.marker, entities, markers);
        // TODO: previously, update was called here
        // TODO: should we still do that?
        let ids = |marker: M| Some(allocator.get_or_create(marker, entities, markers));

        storages
            .deserialize_entity(entity, data.components, ids)
            .map_err(de::Error::custom)
    }
}

<<<<<<< HEAD
<<<<<<< HEAD
/// Wrapper for `Entities` and tuple of `WriteStorage`s that implements `serde::de::Visitor`
struct VisitEntities<'a, 'b: 'a, M: Marker, E, T: Components<M::Identifier, E>> {
    entities: &'a Entities<'b>,
    storages: &'a mut <T as Storages<'b>>::WriteStorages,
    markers: &'a mut WriteStorage<'b, M>,
    allocator: &'a mut Write<'b, M::Allocator>,
    pd: PhantomData<(E, T)>,
}
=======
pub trait IntoDeserialize<M>: Component {
=======
pub trait FromDeserialize<M>: Component {
>>>>>>> e22f637... Finish deserialization overhaul
    /// Serializable data representation for component
    type Data: DeserializeOwned;
>>>>>>> f83d15e... Saveload overhaul

    /// Error may occur during serialization or deserialization of component
    type Error;

    /// Convert this component from a deserializable form (`Data`) using
    /// entity to marker mapping function
    fn from<F>(data: Self::Data, ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(M) -> Option<Entity>;
}

impl<C, M> FromDeserialize<M> for C
where
    C: Component + DeserializeOwned,
{
    type Data = Self;
    type Error = NoError;

    fn from<F>(data: Self::Data, _: F) -> Result<Self, Self::Error> where
        F: FnMut(M) -> Option<Entity> {
        Ok(data)
    }
}

<<<<<<< HEAD
/// Deserialize entities according to markers.
pub fn deserialize<'a, 'de, D, M, E, T>(
    entities: &Entities<'a>,
    storages: &mut <T as Storages<'a>>::WriteStorages,
    markers: &mut WriteStorage<'a, M>,
    allocator: &mut Write<'a, M::Allocator>,
    deserializer: D,
) -> Result<(), D::Error>
where
    M: Marker,
    E: Display,
    T: Components<M::Identifier, E>,
    D: Deserializer<'de>,
{
    deserializer.deserialize_seq(VisitEntities::<M, E, T> {
        entities,
        storages,
        markers,
        allocator,
        pd: PhantomData,
    })
}

/// Struct which implements `DeserializeSeed` to allow serializing
/// components from `World`.
#[derive(SystemData)]
pub struct WorldDeserialize<'a, M: Marker, E, T: Components<M::Identifier, E>> {
    entities: Entities<'a>,
    storages: <T as Storages<'a>>::WriteStorages,
    markers: WriteStorage<'a, M>,
    allocator: Write<'a, M::Allocator>,
=======
/// Wrapper for `Entities` and tuple of `WriteStorage`s that implements `serde::de::Visitor`
struct VisitEntities<'a: 'b, 'b, E, M: Marker, S: 'b> {
    allocator: &'b mut M::Allocator,
    entities: &'b EntitiesRes,
    markers: &'b mut WriteStorage<'a, M>,
    storages: &'b mut S,
>>>>>>> f83d15e... Saveload overhaul
    pd: PhantomData<E>,
}

impl<'de, 'a: 'b, 'b, E, M, S> Visitor<'de> for VisitEntities<'a, 'b, E, M, S>
where
    E: Display,
    M: Marker,
    S: DeserializeComponents<E, M>,
{
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "Sequence of serialized entities")
    }

    fn visit_seq<SEQ>(self, mut seq: SEQ) -> Result<(), SEQ::Error>
    where
        SEQ: SeqAccess<'de>,
    {
        loop {
            let ret = seq.next_element_seed(DeserializeEntity {
                entities: self.entities,
                storages: self.storages,
                markers: self.markers,
                allocator: self.allocator,
                pd: self.pd,
            })?;

            if ret.is_none() {
                break Ok(());
            }
        }
    }
}

macro_rules! deserialize_components {
    ($($comp:ident => $sto:ident,)*) => {
        impl<'b, E, M, $($sto,)*> DeserializeComponents<E, M> for ($($sto,)*)
        where
            E: Display,
            M: Marker,
            $(
                $sto: GenericWriteStorage,
                <$sto as GenericWriteStorage>::Component: FromDeserialize<M>,
                E: From<<
                    <$sto as GenericWriteStorage>::Component as FromDeserialize<M>
                >::Error>,
            )*
        {
            type Data = ($(
                Option<
                    <<$sto as GenericWriteStorage>::Component as FromDeserialize<M>>::Data
                >,)*
            );

            #[allow(unused)]
            fn deserialize_entity<'a, F>(
                &mut self,
                entity: Entity,
                components: Self::Data,
                mut ids: F,
            ) -> Result<(), E>
            where
                F: FnMut(M) -> Option<Entity>
            {
                #[allow(bad_style)]
                let ($(ref mut $sto,)*) = *self;
                #[allow(bad_style)]
                let ($($comp,)*) = components;
                $(
                    if let Some(component) = $comp {
                        $sto.insert(entity, FromDeserialize::<M>::from(component, &mut ids)?);
                    } else {
                        $sto.remove(entity);
                    }
                )*
                Ok(())
            }
        }

        deserialize_components!(@pop $($comp => $sto,)*);
    };
    (@pop) => {};
    (@pop $head0:ident => $head1:ident, $($tail0:ident => $tail1:ident,)*) => {
        deserialize_components!($($tail0 => $tail1,)*);
    };
}

deserialize_components!(
    CA => SA,
    CB => SB,
    CC => SC,
    CD => SD,
    CE => SE,
    CF => SF,
    CG => SG,
    CH => SH,
);
