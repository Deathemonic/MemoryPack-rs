use bumpalo::Bump;
use memorypack::prelude::*;

const PERSON_ID: i32 = 42;
const PERSON_NAME: &str = "Alice";
const PERSON_SCORES: [i32; 3] = [10, 20, 30];

#[derive(MemoryPackable)]
struct OwnedPerson {
    id: i32,
    name: String,
    scores: Vec<i32>
}

#[derive(MemoryPackable)]
#[memorypack(arena)]
struct Person<'arena> {
    id: i32,
    name: &'arena str,
    scores: &'arena [i32]
}

struct ArenaAdapter<'storage>(&'storage Bump);

impl MemoryPackArena for ArenaAdapter<'_> {
    fn allocate<T>(&self, value: T) -> &T { self.0.alloc(value) }

    fn allocate_str(&self, value: &str) -> &str { self.0.alloc_str(value) }

    fn try_allocate_string(
        &self,
        capacity: usize,
        characters: impl IntoIterator<Item = Result<char, MemoryPackError>>
    ) -> Result<&str, MemoryPackError> {
        self.0.try_allocate_string(capacity, characters)
    }

    fn try_allocate_slice<'arena, T: 'arena>(
        &'arena self,
        capacity: usize,
        elements: impl IntoIterator<Item = Result<T, MemoryPackError>>
    ) -> Result<&'arena [T], MemoryPackError> {
        self.0.try_allocate_slice(capacity, elements)
    }
}

fn main() -> Result<(), MemoryPackError> {
    let storage = Bump::new();
    let arena = ArenaAdapter(&storage);
    let person: Person<'_> = {
        let original = OwnedPerson {
            id: PERSON_ID,
            name: PERSON_NAME.to_owned(),
            scores: PERSON_SCORES.to_vec()
        };
        let bytes = MemoryPackSerializer::serialize(&original)?;

        MemoryPackSerializer::deserialize_in(&bytes, &arena)?
    };

    println!("{}: {} {:?}", person.id, person.name, person.scores);

    Ok(())
}
