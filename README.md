
# guessing_game


### Lessons Learned from creating this project:

- Adding carts to the project via the cargo.toml file under the [dependences] and how to import them(bring them into scope) in the project.
- How to get user input using the std library.
- How to use loop, to break out of it and continue running the loop.
- Learned about Operations this is a new way of looking at numbers when they are being compared. There are only three possible outcomes and they are stored in an enum (Less,Greater and Equal)
- Learned how to convert a string into a number only when the number is in string format.
- Learned about the rand crate and how it can be used to create random numbers.
- Learned about traits not 100% sure how they work but here is an interesting analogy of a spell book(the Rng imported from the rand crate ) you don't use
  the spell book but the spells written inside the book. This comes about because of the following reasons
```
use rand::Rng; //the spell book

let secrete_number = thread_rng().gen_range(1..=100);
// at no point is the thread_rng() function imported into this scope but one is able to use it.
// the thread_rng() is known as a trait #the magic spell read out of the Rng(spell book) which is brought into scope.
```
