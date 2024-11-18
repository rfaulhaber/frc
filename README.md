# frc

A library for dealing with dates on the [French Republican Calendar](https://en.wikipedia.org/wiki/French_Republican_calendar) (FRC). 

A port of [my fdate.go library](https://github.com/rfaulhaber/fdate).

This crate provides two implementations of the FRC: one using the Romme Rule, where leap years occur according to the same rules as the Georgian calendar, and one using [Quantum's method](https://quantum5.ca/2022/03/09/art-of-time-keeping-part-4-french-republican-calendar/) for determining the leap year. The latter is more accurate but the former is more popular, and both implementations are provided.
