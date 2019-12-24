#![allow(dead_code)]

use std::io::{self, BufRead};
use std::collections::HashMap;

use std::fs;

#[derive(Debug)]
struct System {
    root: String,

    planets: HashMap<String, Vec<String>>,
}

impl System {
    /// note: this root isn't necessarily the final root (System can be updated)
    fn new(root: &str, child: &str) -> System {
        // initialize a system with one parent and one child
        let mut h = HashMap::new();
        h.insert(root.to_string(), vec!(child.to_string()));
        h.insert(child.to_string(), vec!());

        System {
            root: root.to_string(),
            planets: h,
        }
    }

    fn update(&mut self, parent: &str, child: &str) {
        if child == self.root {
            // update the root if our current root is a child of parent
            self.root = parent.to_string();

            self.planets.insert(parent.to_string(), vec!(child.to_string()));
        } else {

            let o = self.planets.entry(parent.to_string()).or_insert(Vec::new());
            o.push(child.to_string());

            if !self.planets.contains_key(child) {
                self.planets.insert(child.to_string(), Vec::new());
            }
        }
    }

    fn contains(&self, id: &str) -> bool {
        self.planets.contains_key(id)
    }

    fn count(&self, id: &str, base: usize) -> usize {
        // find the count of a planet named id
        let o = self.planets.get(id).unwrap();

        if o.len() == 0 {
           base
        } else {
            o.iter().map(|p| self.count(p, base+1)).sum::<usize>() + base
        }
    }

    fn find_parent(&self, id: &str) -> Option<String> {
        // find the parent of a given ID
        if id == self.root {
            None
        } else {
            for (parent, children) in &self.planets {
                for child in children {
                    if id == child {
                        return Some(parent.to_string());
                    }
                }
            }

            None
        }
    }

    fn path_to_root(&self, id: &str) -> Vec<String> {
        // finds the path from an id to the root (root is last element on vector)
        let mut paths: Vec<String> = Vec::new();
        let mut curr = id.to_string();
        loop {
            if let Some(p) = self.find_parent(&curr) {
                // paths.push(p.clone());
                paths.insert(0, p.clone());
                curr = p.to_string();
            } else {
                break;
            }
        }

        paths
    }
}

fn main() {
    // second approach:
    // add each orbit as its own relationship to a queue
    // then pop from queue by iteratively adding to our system

    let file = fs::File::open("input.txt").unwrap();

    let mut orbits: Vec<(String, String)> = Vec::new();

    for line in io::BufReader::new(file).lines() {
        let l = line.unwrap();
        let o: Vec<&str> = l.split(")").collect();

        orbits.push((o[0].to_string(), o[1].to_string()));
    }

    let planet = orbits.pop().unwrap();
    let mut sys = System::new(&planet.0, &planet.1);

    // this whole workflow is disgusting
    let mut curr = 0;
    while orbits.len() > 0 {
        if sys.contains(&orbits[curr].0) || sys.contains(&orbits[curr].1) {
            sys.update(&orbits[curr].0, &orbits[curr].1);
            orbits.remove(curr);
        }
        if orbits.len() == 0 { break; }
        curr = (curr+1) % orbits.len();
    }

    // println!("{:?}", orbits);
    println!("{}", calc(sys));
}

fn calc(sys: System) -> usize {
    // calculate number for orbital transfers from YOU -> SAN
    let mut path_you = sys.path_to_root("YOU");
    let mut path_san = sys.path_to_root("SAN");

    loop {
        if path_you[0] == path_san[0] {
            path_you.remove(0);
            path_san.remove(0);
        } else {
            break;
        }
    }

    // println!("{:?}", path_you);
    // println!("{:?}", path_san);

    path_you.len() + path_san.len()
}
