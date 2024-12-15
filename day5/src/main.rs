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

    let jobs = puzzle
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|page| {
                    //dbg!(page);
                    page.parse().expect("Pages must be numbers")
                })
                .collect()
        })
        .collect();

    // _part_1(jobs, &rule_lookup);
    part_2(jobs, &rule_lookup);
}

fn _part_1(jobs: Vec<Vec<usize>>, rule_lookup: &HashMap<usize, Vec<usize>>) {
    let valid_jobs: Vec<Vec<usize>> = jobs
        .into_iter()
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

fn part_2(jobs: Vec<Vec<usize>>, rule_lookup: &HashMap<usize, Vec<usize>>) {
    let fixed_jobs: Vec<Vec<usize>> = jobs
        .into_iter()
        .filter(|job| !is_job_valid(job, &rule_lookup))
        .map(|job| fix_job(&job, rule_lookup))
        .collect();

    let mids: Vec<usize> = fixed_jobs.iter().map(|job| job[job.len() / 2]).collect();

    dbg!(/*fixed_jobs,*/ mids.clone().iter().sum::<usize>());
}

fn fix_job(job: &Vec<usize>, rule_lookup: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    // dbg!("fix_job", job, rule_lookup);
    let mut job_remaining = job.clone();
    let mut fixed_job: Vec<usize> = Vec::new();
    while !job_remaining.is_empty() {
        let next_insert = job_remaining
            .iter()
            .filter(|page| {
                let already_inserted = fixed_job.contains(page);
                let requirements = rule_lookup.get(page);
                if let Some(requirements) = requirements {
                    let mut requirements_met = true;
                    for requirement in requirements {
                        if job.contains(requirement) && !fixed_job.contains(requirement) {
                            requirements_met = false;
                        }
                    }

                    requirements_met && !already_inserted
                } else {
                    !already_inserted
                }
            })
            .collect::<Vec<&usize>>();

        let next_insert = **next_insert.clone().first().unwrap();
        fixed_job.push(next_insert);

        job_remaining.remove(
            job_remaining
                .clone()
                .iter()
                .position(|page| *page == next_insert)
                .unwrap(),
        );
    }

    fixed_job
}
