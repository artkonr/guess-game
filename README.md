# guess-game
Trivial stdin-based guess-the-number game implemented in Rust.

To play, simply start the application using ``cargo run`` or build an executable to run it afterwards using ``cargo build --release``.

Application will have you guessing random numbers from 1 to whatever upper bound you supply. It will expect input in the console from you and will respond with a simplistic success/failure message judging by the result of the guess.

If you feel like you're done with this nonsense and you have better things to do than hear from cmd that you're not as lucky as you'd like it think you are, just print ``quit`` in the console, and the game will terminate. Once the game receives a termination signal, it will shut down and display the game statistics.

The game also supports other commands, such as:

* ``author`` -- will show information about the creator of the game
* ``hi`` -- greets the player
* ``kill`` -- terminates the game at once
* ``show`` -- prints the statistics of the running game
* ``quit`` -- prints the statistics of the running game and then quits
* ``version`` -- prints the version of the game
  
Have fun!
