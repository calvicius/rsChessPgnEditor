use regex::Regex;

pub fn root_node() -> String {

    "1z".to_string()
}


pub fn get_next_mainline_indx(node_indx: String) -> String {
    
    let re = Regex::new(r"(\d+)z$");
    let matched = re.unwrap();
    let value_opt = matched.captures(&node_indx);

    if value_opt.is_some() {
        let val = value_opt.unwrap();
        if val.get(1).is_none() {
            return format!("{}{}", node_indx, "1z");
        }
        else {
            let inner_re = Regex::new(r"\d+z$");
            let c1 = inner_re.unwrap().replace(&node_indx, "");
            let c2 = val.get(1).unwrap().as_str();
            let c2int = c2.parse::<i32>().unwrap() + 1;
            let c3 = "z";
            return format!("{}{}{}", c1, c2int, c3);
        }
    }
    else {
        return format!("{}{}", node_indx, "1z");
    }

}


pub fn get_next_sibling_indx(node_indx: String, num_sibling: i32) -> String {

    if num_sibling == 0 {
        return get_next_mainline_indx(node_indx);
    } else {
        let ret = format!("{}{}{}", node_indx, num_sibling, "n");
        return ret;
    }
}


pub fn get_child_indx(node_indx: String) -> i32 {
    
    let re = Regex::new(r"(?P<num>\d+)n$");
    let matched = re.unwrap();
    let caps = matched.captures(&node_indx);
    
    if caps.is_some() {
        let idx = &caps.unwrap()["num"].to_string().parse::<i32>().unwrap();
        return *idx;
    }
    else {
        // by convention, mainline has childIndx 0
        return 0;
    }
}


pub fn is_absolute_mainline(node_indx: String) -> bool {

    let re = Regex::new(r"^\d+z$");
    if re.is_ok() {
        if re.unwrap().is_match(&node_indx)  {
            return true;
        }
        else {
            return false;
        }
    }
    else {
        return false;
    }
}


pub fn get_branch_node(selected_node: String) -> Option<String> {

    if !selected_node.contains("n") {
        // branch is already mainline
        return None;
    }

    // strip trailing zeros
    let mut parent: String = "".to_string();
    let stem: String;
    
    let re = Regex::new(r"\d+z$");
    if re.is_ok() {
        let tmp = re.unwrap();
        let tmp1 = tmp.replace(&selected_node, "");
        parent = tmp1.to_owned().to_string();
    }
    
    let re = Regex::new(r"\d+n$");
    //if re.is_ok() {
        //stem = &re.unwrap().replace(&parent, "");
        let tmp = re.unwrap();
        let tmp1 = tmp.replace(&parent, "");
        stem = tmp1.to_owned().to_string();
    //}
    
    // replace last index with mainline index
    if stem.ends_with("n") {
        //
        let re = Regex::new(r"\d+n$");
        //if re.is_ok() {
            let tmp1 = &re.unwrap().replace(&parent, "1z");
            return Some(tmp1.to_string());
        //}
    } 
    else {
        let re = Regex::new(r"(\d+)z$");
        let value_opt = re.unwrap().captures(&stem);
        // consider this stem: 11z
        // get(0) == 11z ; get(1) == 11
        let valregex = value_opt.unwrap().get(1).unwrap().as_str();
        let valnum: String = valregex.to_string();
        let mainline_indx: i32 = valnum.parse::<i32>().unwrap();
        
        let re = Regex::new(r"(\d+)z$");
        let tmp1 = format!("{}{}", (mainline_indx + 1), "z");
        let repl = &re.unwrap().replace(&stem, tmp1);

        return Some(repl.to_string());
    }
    
}