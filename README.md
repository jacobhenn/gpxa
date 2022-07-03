# gpxa

`gpxa` is a GPX (GPS Exchange) file analyzer. GPX files store recorded (or planned) positions and paths. apps like Strava record GPX files and provide analyses of them. `gpxa` is an open-source alternative to these kinds of services. as for recording a gpx file to analyze, check out merlos' [Open GPX Tracker](https://github.com/merlos/iOS-Open-GPX-Tracker) (iOS only).

```
Usage: gpxa <path> [-t <track>] [-p <pretty>] [-d <dist-units>] [-s <speed-units>] [-o <output>]

A GPX (GPS Exchange) file analyzer.

Positional Arguments:
  path              a path to a gpx file

Options:
  -t, --track       which track in the gpx file to analyze (0-based index). not
                    necessary if there is only one track.
  -p, --pretty      when to print stat names and units. values: always, auto,
                    never. auto detects if output is being piped. defaults to
                    auto.
  -d, --dist-units  distance units to use. ft for feet and m for metres.
                    defaults to metres.
  -s, --speed-units speed units to use. values: km/h, mi/h, m/s, ft/s, min/km,
                    min/mi. defaults to mi or km per hr depending on
                    --dist-units.
  -o, --output      what to display in output. comma-separated list of values.
                    values: total-dist, total-time, avg-speed, median-speed,
                    max-elev, min-elev. order of values does not affect order of
                    output. defaults to all of them.
  --help            display usage information
```

## examples

```
$ gpxa test.gpx
total distance: 2384.11 m
total time: 32m9s
average speed: 4.45 km/h
median speed: 4.30 km/h

max elevation: 78.70 m
min elevation: 65.60 m
```

```
$ gpxa test.gpx > out.txt
$ cat out.txt
2384.1084945318867
1929000
4.449347112656709
4.297140706187874
78.7
65.6
```

## output

- total distance
    - the total distance traveled along the track from start to finish.
- total time
    - the total time elapsed from the start of the track to the end.
- average speed
    - the total distance divided by the total time.
- median speed
    - the [weighted median](https://en.wikipedia.org/wiki/Weighted_median) of all sampled speeds. this is the most robust measure of the recorded "typical speed".
- max elevation
    - the largest recorded elevation
- min elevation
    - the smallest recorded elevation

### pretty printing

- if `--pretty` is set to `auto`, it will only pretty print if the output of `gpxa` is not being redirected.
- when pretty printing is off, descriptions, blank lines, and units will not be printed, all units are strict (e.g. won't auto-convert 2000 ft to 0.37 mi), and times are given in integer numbers of milliseconds. in addition, errors while finding any requested statistic will prevent the printing of any statistics and behave as a fatal error instead of printing in stdout with the successfully computed statistics.

## known errors / restrictions

- in order to get all statistics, the given track should have at least 3 waypoints.
