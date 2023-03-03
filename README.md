# chess-rs

The system is currently playable with some missing features. (See [TODO](https://github.com/hokkonsz/chess-rs/edit/main/README.md#todo "TODO"))

### Game Controller

The system can be controller through the game controller function. It takes two inputs Position 1 and Position 2. At Position 1 we should find a valid Unit, which we want to move to Position 2 to the Target Position. Then the controller evaluates the Unit's Type and the Target Position and mutates the board based on the results.

### Chess Space to Array Space

For user-friendly usability we can produce Positions from Chess Notations.

![board_positions](https://user-images.githubusercontent.com/54407548/222514238-07e80059-bf42-41af-8fa0-caa919778771.png)

For example, the .to function produces a vector, which contains all the Positions between two Positions excluding the starting and ending Positions.

![fn_to](https://user-images.githubusercontent.com/54407548/222519034-80bf5fd8-3aac-49d1-ba7d-6fd92fa4b655.png)

### Unit Step

To validate a Step we are using basic math, where we substract the Unit's Position from the Target's Position. Furthermore there are additional conditions to each Step. The whole Step System is build around the Type State Pattern, which in my mind really fits the process. (Or maybe I just really wanted to try my hands on this pattern.)

There are 4 differen States to a Step.

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

![pawn1](https://user-images.githubusercontent.com/54407548/222733817-f5a123cb-bff9-4d3f-a3bb-df5285629359.png)
![pawn2](https://user-images.githubusercontent.com/54407548/222733874-f24068a6-9f42-4789-a47f-3b12ec12fa51.png)

#### Bishop

![bishop](https://user-images.githubusercontent.com/54407548/222734732-9f297558-9a30-49b9-93c1-f3a34f80a2a4.png)

#### Knight

![knight](https://user-images.githubusercontent.com/54407548/222733908-1070d758-a48d-4416-b58c-e35dd2ea42f6.png)

#### Rook

![rook](https://user-images.githubusercontent.com/54407548/222734704-822fd637-18ba-4730-8114-c703460fc85b.png)

#### Queen

![queen](https://user-images.githubusercontent.com/54407548/222734634-ca94a7c9-cf55-4986-a77a-102419eceef1.png)

#### King

![king1](https://user-images.githubusercontent.com/54407548/222734566-17bd67bf-4162-4017-bcfc-a82e0bcf61f7.png)
![king2](https://user-images.githubusercontent.com/54407548/222734582-b2a28bb2-18c8-46b4-852c-cb94b8f33a21.png)

### Examples

Currently there are two examples to test out the system, one of them is using eGUI and the other one is using notan. Right now I'm more focused on notan, which I can gladly recommend to anyone, who wants to quickly implement and test out things in Rust.

### TODO

List of missing features and future plans.

* CheckMate / StaleMate
* Promote Unit
