# rust-serialization-benchmark

A port of [`python-serialization-benchmark`](https://github.com/voidfiles/python-serialization-benchmark) to Rust, using [`serde_json`](https://github.com/serde-rs/json) as the serializer. 

I attempted to reproduce the workings of the Python objects as closely as possible, and the output of both the Python and Rust serializers is identical.

## Running the Rust benchmark

Clone this repo, and run
```
cargo bench
```
The results from `cargo bench` will be in nanoseconds, rather than seconds as in the Python benchmark. 


## Results
Running both the Python and Rust benchmarks on my local machine (a [late-2013 MacBook Pro with a 2.6GHz quad-core Intel Core i7](https://support.apple.com/kb/sp690?locale=en_US)), produced the following:


```
Library                  Many Objects (seconds)    One Object (seconds)    Relative
---------------------  ------------------------  ----------------------  ----------
serde_json (Rust)                    0.00407219              0.00218101     0.48
serpyco                              0.00872946              0.00450587     1
Custom                               0.010844                0.00496244     1.19426
lima                                 0.0138385               0.00684261     1.56257
Pickle                               0.015667                0.016381       2.4214
serpy                                0.0538666               0.0283134      6.20914
Strainer                             0.0799131               0.0385959      8.95399
Toasted Marshmallow                  0.104154                0.0554521     12.0591
Colander                             0.280941                0.142262      31.9752
Avro                                 0.442745                0.217734      49.9027
Lollipop                             0.4566                  0.223092      51.3544
Marshmallow                          0.609148                0.307631      69.2675
kim                                  0.799352                0.388839      89.7742
Django REST Framework                0.724293                0.543168      95.7634
```
Each framework is asked to serialize a list of 2 objects a 1000 times, and then 1 object a 1000 times. See the [original Python benchmark](https://voidfiles.github.io/python-serialization-benchmark/) for more details.