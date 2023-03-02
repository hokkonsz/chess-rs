# chess-rs

The system is currently playable with some missing features. (See [TODO](https://github.com/hokkonsz/chess-rs/edit/main/README.md#todo "TODO"))

### Game Controller

The whole system functioning through the game controller. It takes two inputs Position 1 and Position 2. At Position 1 we should find a valid Unit, which we want to move to Position 2, to the Target Position. The controller evaluates the Unit and the Target Positions and mutates the board based on the results.

### Chess Space to Array Space

For user-friendly usability we can produce Positions from Chess Notations.

![board_positions](https://user-images.githubusercontent.com/54407548/222514238-07e80059-bf42-41af-8fa0-caa919778771.png)

For example, the .to function produces a vector, which contains all the Positions between two Positions excluding the starting and ending Positions.

![fn_to](https://user-images.githubusercontent.com/54407548/222519034-80bf5fd8-3aac-49d1-ba7d-6fd92fa4b655.png)

### Unit Step

To calculate each Step we are using basic math, where we substract the Unit's Position from the Target Positions. Furthermore there are additional conditions to each Step

![pawn1](https://user-images.githubusercontent.com/54407548/222507843-4c2b0444-4f25-4a0c-97e4-d6ae3d8422b1.png)
![pawn2](https://user-images.githubusercontent.com/54407548/222508482-ca5d858c-37f0-4003-92eb-f05cf7c6861f.png)

### Examples

Currently there are two examples to test out the system, one of them is using eGUI and the other one is using notan. Right now I'm more focused on notan, which I can easily recommend to anyone, who wants to quickly implement and test stuff.

### TODO

There are some missing features and future plans.

* CheckMate / StaleMate
* Promote Unit
