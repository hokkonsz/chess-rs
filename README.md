
# ♟ chess-rs

[![dependency status](https://deps.rs/repo/github/hokkonsz/chess-rs/status.svg)](https://deps.rs/repo/github/hokkonsz/chess-rs)

A simple chess system written in Rust. The project is still in work in progress, you can find the missing features below. (See [TODO](https://github.com/hokkonsz/chess-rs/edit/main/README.md#todo "TODO"))

### Game Controller

The system can be controller through the game controller function. It takes two inputs Position 1 and Position 2. At Position 1 we should find a valid Unit, which we want to move to Position 2 to the Target Position. Then the controller evaluates the Unit's Type and the Target Position and mutates the board based on the results.

### Chess Space to Array Space

For user-friendly usability we can produce Positions from Chess Notations.

![board_positions](https://user-images.githubusercontent.com/54407548/222514238-07e80059-bf42-41af-8fa0-caa919778771.png)

For example, the .to function produces a vector, which contains all the Positions between two Positions excluding the starting and ending Positions.

![fn_to](https://user-images.githubusercontent.com/54407548/227720806-6289e3a0-cb82-4114-b210-20e116476e9f.png)

### Unit Step

To validate a Step we are using basic math, where we substract the Unit's Position from the Target's Position. Furthermore there are additional conditions to each Step. The whole Step System is build around the Type State Pattern, which in my mind really fits the process. (Or maybe I just really wanted to try my hands on this pattern.)

There are 3 different States to a Step.

ConditionState

  - Where we can add Conditions like Unit is not a King at Position or Unit is an Enemy at Position.
  - We can also add Actions like Move/Remove. This can be usefull for special cases like En passant or Castling, where we want to do more than one thing.
  
ResultState

  - This state holds information about the evaluation process.
  - In the ResultState we can execute the Actions.

ImageState

  - This State holds information about the former state of the board, so we can reconstruct the old state if necessary.

![image](https://user-images.githubusercontent.com/54407548/222757554-b7e2d55b-bfaa-4bf3-acfe-6484504c4c10.png)

The pictures below show the calculations for each unit.

#### Pawn

![pawn1](https://user-images.githubusercontent.com/54407548/222778356-2c743059-a499-40d6-bd10-241bec08502b.png)
![pawn2](https://user-images.githubusercontent.com/54407548/222778371-fa3129c1-d744-41ef-b72a-fae912c1e0cf.png)

#### Bishop/Rook

![bishop](https://user-images.githubusercontent.com/54407548/222780200-4f19aac3-d079-4867-bf85-ba1659031cbb.png)
![rook](https://user-images.githubusercontent.com/54407548/222780213-85fd7dcb-4304-4896-b6ce-ecc2309f9473.png)

#### Knight/Queen

![knight](https://user-images.githubusercontent.com/54407548/222780243-61bf9e0d-1b8e-496e-b717-7ee68c579b43.png)
![queen](https://user-images.githubusercontent.com/54407548/222780263-1e5df148-191c-44a3-9c6f-51ced47632f6.png)

#### King

![king1](https://user-images.githubusercontent.com/54407548/222780674-704aa34f-d09b-4032-bff7-7bb0c6a2573b.png)
![king2](https://user-images.githubusercontent.com/54407548/222780691-7391bbf3-15c9-455e-bf1e-0ae1ff6859ed.png)

### Examples

Currently there are two examples to test out the system, one of them is using eGUI and the other one is using notan. Right now I'm more focused on notan, which I can gladly recommend to anyone, who wants to quickly implement and test out things in Rust.

```rust
cargo run --example app_notan
```

or

```rust
cargo run --example app_egui --features egui
```

### TODO

List of missing features and future plans.

* CheckMate / StaleMate
* Promote Unit
