# Misc notes

- dyn vs impl
    - learn more
    - try to use more impl here
- View not fully encapsulated - as tick function is in main::Presenter
- test tunnel creation disabled after refactoring...
    - test_map_builder.rs
    -

# Book

- page 455

# To Discuss

- how to better organize model
    - model.rs has lots of infos in it....
    - maybe have the same mod module name for all e.g. map map_builder
- lots of files in model folder need use crate:: --- why ??
    - could not get to work with super nor self
    - e.g. map_builder.rs
- test in folder model have absolute part to crate:: in use statements.. is that necessary??
- game_logic.rs could be model.rs but then some warning came...