# terradle-solver
An algorithm to solve [https://www.terradle.com/](terradle.com).

The algorith determines the guess with the lowest average (arithmetic mean) amount of possible weapons for the next guess, based on all possible sets of new information it could provide.

Finding the weapon of the day should take ~2.659 attempts in average.
| Range   | Count |
|---------|-------|
| 1 | 1     |
| 2.0 - <2.5 | 172 |
| 2.5 - <3.0 | 2   |
| 3.0 - <3.5 | 160 |
| 3.5 - <4.0 | 6   |
| 4.0 - <4.5 | 16  |
| 4.5 - <5.0 | 3   |
| 5.0 - <5.5 | 6   |
| 5.5 - <6.0 | 4   |
| 6.0 | 1   |

If you are wondering about the meaning of the .5 values:
Sometimes, after an incorrect guess, there are multiple items remaining with the same stats. In this case their amount of attempts are averaged (example: assume after 3 tries you get a choice between two weapons. Equivalently to averaging, you could say one weapon takes 3 tries while the other weapon takes 4).

Dependencies:
- openssl header files

alternatively you can modify Cargo.toml to use rustls instead of native-tls

Download and build the project:
```
git clone https://github.com/Stachelbeere1248/terradle-solver.git
cd terradle-solver
cargo build -r
```
Interactively solve the terradle of the day:
```
./terradle-solver
```
Print a ranked list of all the openers:
```
./tarradle-solver --mode openers
```
List the amount of attempts the solver needs to determine the stats of every weapon. Note that it counts until it has found out the exact stats of the item, rather than the item itself. Due to luck, this results in slightly higher try-counts than an actual terradle would take, while not counting additional "random" tries when items have same stats.
```
./terradle-solver --mode simulate # | sort
```

![cli-example](example.png)
![web-example](example-web.png)
