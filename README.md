Written in rust, this a port of [sphere-knn](https://github.com/darkskyapp/sphere-knn), an api that provides fast nearest-neighbor lookups on a sphere. This is useful if, for example, you have a database of geographic points (latitude, longitude) and want to swiftly look up which of those points are near a given latitude, longitude pair. 


## Rust Usage

For an example of how to use, you can peruse the [api example](./crates/sphere-knn/examples/api.rs) or run it from the cli 

```
cd crates/sphere-knn/examples && cargo run --example api 
```

The library needs to know how to find the latitude / longitude keys on your object, so you will need to implement `SphereKnnGetters` on your data object. 

```
use sphere_knn::{run, Opts, SphereKnnGetters};


impl<'name> SphereKnnGetters for ExampleStruct<'name> {
    fn get_lat(&self) -> f64 {
        return self.latitude;
    }
    fn get_lng(&self) -> f64 {
        return self.longitude;
    }
}
```

Once defined, you initialize the library with a vector of data, the result of which is an invocable function 

```
let find_nearest_fn = sphere_knn::run(data);
```

This holds the vector-as-a-tree and allows for nearest neighborhood lookup on subsequent usage 

```

let hartford = ExampleStruct {
        latitude: 41.76,
        longitude: -72.67,
        name: "hartford",
};

let result = find_nearest_fn(
    hartford.get_lat(),
    hartford.get_lng(),
    Opts {
        max_distance_threshold_meters: Some(200_000.0),
        number_results: Some(1 as usize),
    },
);
```