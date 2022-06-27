# gpxa

`gpxa` is a GPX (GPS Exchange) file analyzer. GPX files store recorded (or planned) positions and paths. apps like Strava record GPX files and provide analyses of them. `gpxa` is an open-source alternative to these kinds of services. as for recording a gpx file to analyze, check out merlos' [Open GPX Tracker](https://github.com/merlos/iOS-Open-GPX-Tracker) (iOS only).

```
Usage: gpxa <path> [-t <track>] [-u <dist-units>] [-v <speed-units>]

A GPX (GPS Exchange) file analyzer.

Positional Arguments:
  path              a path to a gpx file

Options:
  -t, --track       which track in the gpx file to analyze. not necessary if
                    there is only one track.
  -u, --dist-units  distance units to use. ft for feet and m for metres.
                    defaults to metres.
  -v, --speed-units speed units to use. values: km/h, mi/h, m/s, ft/s, min/km,
                    min/mi. defaults to (mi|km)/hr depending on --dist-units.
  --help            display usage information

```
