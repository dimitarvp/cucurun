use cucumber::{after, before, CucumberBuilder, DefaultOutput};
use std::path::Path;

pub struct MyWorld {
    // You can use this struct for mutable context in scenarios.
    foo: String,
}

impl cucumber::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        // This function is called every time a new scenario is started
        MyWorld {
            foo: "a default string".to_string(),
        }
    }
}

pub mod example_steps {
    use cucumber::steps;

    // Any type that implements cucumber::World + Default can be the world
    steps!(crate::integration_tests::MyWorld => {
        given "I am trying out Cucumber" |world, step| {
            world.foo = "Some string".to_string();
            // Set up your context in given steps
        };

        when "I consider what I am doing" |world, step| {
            // Take actions
            let new_string = format!("{}.", &world.foo);
            world.foo = new_string;
        };

        then "I am interested in ATDD" |world, step| {
            // Check that the outcomes to be observed have occurred
            assert_eq!(world.foo, "Some string.");
        };

        then regex r"^we can (.*) rules with regex$" |world, matches, step| {
            // And access them as an array
            assert_eq!(matches[1], "implement");
        };

        then regex r"^we can also match (\d+) (.+) types$" (usize, String) |world, num, word, step| {
            // `num` will be of type usize, `word` of type String
            assert_eq!(num, 42);
            assert_eq!(word, "olika");
        };

        then "we can use data tables to provide more parameters" |world, step| {
            let table = step.table().unwrap().clone();

            assert_eq!(table.header, vec!["key", "value"]);

            let expected_keys = table.rows.iter().map(|row| row[0].to_owned()).collect::<Vec<_>>();
            let expected_values = table.rows.iter().map(|row| row[1].to_owned()).collect::<Vec<_>>();

            assert_eq!(expected_keys, vec!["a", "b"]);
            assert_eq!(expected_values, vec!["fizz", "buzz"]);
        };
    });
}

// Declares a before handler function named `a_before_fn`
before!(a_before_fn => |scenario| {

});

// Declares an after handler function named `an_after_fn`
after!(an_after_fn => |scenario| {

});

// A setup function to be called before everything else
fn setup() {}

pub fn run_tests() {
    let output = DefaultOutput::default();
    let mut builder = CucumberBuilder::new(output);
    let steps = example_steps::steps();
    builder
        .features(vec![Path::new("./features").to_path_buf()])
        .steps(steps)
        .setup(setup)
        .before(vec![a_before_fn])
        .after(vec![an_after_fn]);
    builder.run();
}
