# rusty_docs 0.2
An automated doc generator for front end applications

## The scenario
Have you been workig on a huge application, either your own or an api for work, and you felt a little *rusty* on what does what. You or your team just has not had the time for making documentation for your api because everyone is working at 1001% efficency? Well then `rusty_docs` s for you. Written it rust, `rusty_docs` needs only a `.json` to kick things off. More documentation files will be coming soon.

## The mindset
`rusty_docs` is meant to be modular and causomizeable. So if you need to implement something `rusty_docs` uses `impl gtsx::GenerateTsx` to document things like functions, parameters, classes and errors. So if you have a data structure that is not implemented, rusty docs has your back.

## Getting started

To install the library, run: 

```
cargo install --git https://github.com/adamkali/rusty_docs.git
```

Then you will need is a `documentation.json` and a `main.rs` file ( or any rust file really ). Here are some examples

### `documentation.json`
```
{
    'name': 'Test',
    'params': {
        'name': 'Foo'
        'type_of': 'String',
        'explanation': 'Runs the test with some foo guid'
    },
    'explanation': 'checks if a guid is valid.'
}
```
### `main.js`
```
TODO
```
