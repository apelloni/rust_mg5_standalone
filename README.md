# rust_mg5_standalone
Bridge MadGraph standalone library to be used directly in rust

## Dependencies
 - [MadGraph5_aMC@NLO](https://launchpad.net/mg5amcnlo)
 - python3
 - `cxx` crate for rust

In order to run Madgraph one need a minimal virtual enviroment
```
python -m venv .venv
source .venv/bin/activate
pip instal six
```

Different processes can be accessed by writing the corresponding card provided that
**the card has the same name as the folder!**
```
make -e PROCESS=standalone_uubar_aag_ddbar
```
