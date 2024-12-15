use std::collections::{HashMap, HashSet};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filepath = &args[1];

    let puzzle = std::fs::read_to_string(filepath).expect("File to open");

    let rules: Vec<(usize, usize)> = puzzle
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split('|').collect();

            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect();

    let mut rule_lookup: HashMap<usize, Vec<usize>> =
        rules.iter().map(|rule| (rule.1, Vec::new())).collect();

    for (k, v) in rules.clone() {
        let mut list = rule_lookup.get(&v).unwrap().clone();
        list.push(k);
        rule_lookup.insert(v, list);
    }

    let valid_jobs: Vec<Vec<usize>> = puzzle
        .lines()
        .skip(rules.len() + 1)
        .map(|line| {
            line.split(',')
                .map(|page| {
                    //dbg!(page);
                    page.parse().expect("Pages must be numbers")
                })
                .collect()
        })
        //.take(1)
        .filter(|job| is_job_valid(job, &rule_lookup))
        .collect();

    let mids: Vec<usize> = valid_jobs.iter().map(|job| job[job.len() / 2]).collect();

    dbg!(mids.clone().iter().sum::<usize>() /*, valid_jobs*/);
}

fn is_job_valid(job: &Vec<usize>, rule_lookup: &HashMap<usize, Vec<usize>>) -> bool {
    let mut read_pages: HashSet<usize> = HashSet::new();
    for page in job {
        let required_before = rule_lookup.get(page);
        if let Some(x) = required_before {
            let relevant_requirements = x.iter().filter(|p| job.contains(p));
            //dbg!(&relevant_requirements, &read_pages, page);
            for required_page in relevant_requirements {
                if !read_pages.contains(required_page) {
                    //println!("Page {page} is invalid because {required_page} has not been read.");
                    return false;
                }
            }
        }

        //println!("Adding page {page} to hashset as read");
        read_pages.insert(*page);
        //dbg!(&read_pages, read_pages.contains(page));
    }

    true
}
