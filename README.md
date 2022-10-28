Pick chess openings at random.

The usage is as follows:
``` shell
$ opening-roulette --help
Picks chess openings at random for you

Usage: opening-roulette <OPENING_TABLE>

Arguments:
  <OPENING_TABLE>  File containing all openings including their weights

Options:
  -h, --help  Print help information
```

With an opening file like:
``` toml
[white]
"Scotch Game" = 1
"Ruy Lopez" = 1
"King's Gambit" = 1

[black_e4]
"Berlin Defense" = 1
"Sicilian Dragon" = 1
"Sicilian Najdorf" = 1
"French Defense" = 1

[black_d4]
"King's Indian Defense" = 1
"Queen's Gambit Declined" = 1
```

The roulette picks a radom opening for you:
``` shell
$ opening-roulette openings.toml
white: Scotch Game
against 1.e4: French Defense
against 1.d4: King's Indian Defense
```

