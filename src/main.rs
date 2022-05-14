const ELEMENT_COUNT: usize = 6;
const GROUP_SIZE: usize = 2;

fn check_group(group: &[usize]) -> bool {
    assert_eq!(group.len(), GROUP_SIZE);

    // Manually specified grouping rules for now
    if group[0] == 1 && group[1] == 6 {
        return false;
    } else if group[0] == 2 && group[1] == 4 {
        return false;
    } else if group[0] == 4 && group[1] == 5 {
        return false;
    } else {
        return true;
    }
}

fn generate_groups(elements: &mut[usize], index: usize) {
    if index == elements.len() {
        let mut i = 0;
        while i < elements.len() {
            print!("{:?}", &elements[i..i+GROUP_SIZE]);
            i += GROUP_SIZE;
        }
        println!();
        return;
    }

    for i in index..elements.len() {
        if index % GROUP_SIZE != 0 && elements[i] < elements[index - 1] {
            // Skip groups that aren't internally sorted
            continue
        } else if index > 0 && index % GROUP_SIZE == 0 && elements[i] < elements[index - GROUP_SIZE] {
            // Skip groups that aren't sorted by their first memeber
            continue
        }

        (elements[index], elements[i]) = (elements[i], elements[index]);
        if index % GROUP_SIZE == (GROUP_SIZE - 1) && !check_group(&elements[index-(GROUP_SIZE-1)..index+1]) {
            // Skip groups that fail the constraints
        } else {
            generate_groups(elements, index + 1);
        }
        (elements[index], elements[i]) = (elements[i], elements[index]);
    }
}

fn main() {
    let mut elements: Vec<usize> = (1..ELEMENT_COUNT+1).collect();
    
    println!("Running with {} elements and groups of {}", ELEMENT_COUNT, GROUP_SIZE);
    generate_groups(elements.as_mut(), 0);
}
