#![warn(clippy::all)]

use std::{collections::{LinkedList, HashMap}};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq,Clone)]
enum InodeType{
    Directory,
    File
}
impl Default for InodeType{
    fn default() -> Self {
        InodeType::Directory
    }
}

#[derive(Debug,Default,PartialEq, Eq,Clone)]
struct InodeRec{
    inode_type: InodeType,
    name: String,
    size: Option<u64>,
    children: HashMap<String,InodeRec>
}
impl InodeRec{
    fn insert_at(&mut self, path: LinkedList<String>, inode: InodeRec){
        let mut path_clone = path.clone(); 
        let me = path_clone.pop_back();
        match me {
            Some(_) => {
                match path_clone.back() {
                    Some(child) => {
                        self.children
                        .get_mut(child)
                        .unwrap()
                        .insert_at(path_clone, inode);
                    },
                    None => {if !self.children.contains_key(&inode.name) {self.children.insert(inode.name.to_string(), inode);}},
                }
            },
            None => todo!(),
        }
    }

    fn print(&self, indentation:usize){
        for _ in 0..indentation{
            print!("  ");
        }
        println!("- {} ({}{})",self.name, 
            if self.inode_type == InodeType::Directory {"dir"} else {"file"},
            if self.size.is_some() {", size=".to_string()+&self.size.unwrap().to_string()} else {"".to_string()}
        );
        for (_, child) in &self.children{
            child.print(indentation+1);
        }
    }
    fn compute_size(&mut self) -> u64{
        if self.inode_type == InodeType::File{
            return self.size.unwrap();
        }else{
            self.size = Some(if self.children.len()>0 { self.children.values_mut().map(|c|c.compute_size()).sum() } else {0});
            return self.size.unwrap();
        }
    }
    fn retrieve_folders_from_size(&self, folders: &mut Vec<InodeRec>, threshold:u64){
        if self.inode_type == InodeType::Directory && self.size.unwrap()<=threshold{
            folders.push(self.clone());
        }
        for (_, child) in &self.children{
            child.retrieve_folders_from_size(folders, threshold);
        }
    }
    fn retrieve_folders_from_size_free(&self, folders: &mut Vec<InodeRec>, threshold:u64){
        if self.inode_type == InodeType::Directory && self.size.unwrap()>=threshold{
            folders.push(self.clone());
        }
        for (_, child) in &self.children{
            child.retrieve_folders_from_size_free(folders, threshold);
        }
    }
}

fn main(){
    let input = include_str!("input.txt");

    let mut current_path :LinkedList<String> = Default::default();
    current_path.push_front("/".to_string());
    let mut inoderoot :InodeRec = Default::default();
    inoderoot.name = "/".to_string();

    let mut listing = false;
    for line in input.lines(){
        if line.trim() == "$ ls" {
            listing = true;
        }
        let cd_command = sscanf::sscanf!(line.trim(), "$ cd {str}");
        if cd_command.is_ok() {
            listing = false;
            let folder = cd_command.unwrap();
            match folder {
                "/" => {current_path.clear(); current_path.push_front("/".to_string());}
                ".." => {
                        if current_path.len() == 1 {
                            current_path.clear(); 
                            current_path.push_front("/".to_string());
                        }else{
                            current_path.pop_front();
                        }
                    }
                _ => {
                    inoderoot.insert_at(current_path.clone(), InodeRec { inode_type: InodeType::Directory, name: folder.to_string(), size: Default::default(), children: Default::default() });
                    current_path.push_front(folder.to_string());
                }
            }
        }
        let ls_item = sscanf::sscanf!(line.trim(), "{str} {str}");
        if ls_item.is_ok() && listing{
            let (item_type, item_name) = ls_item.unwrap();
            if item_type == "dir"{
                inoderoot.insert_at(current_path.clone(), InodeRec { inode_type: InodeType::Directory, name: item_name.to_string(), size: Default::default(), children: Default::default() });
            }else if item_type.parse::<u64>().is_ok(){
                inoderoot.insert_at(current_path.clone(), InodeRec { inode_type: InodeType::File, name: item_name.to_string(), size: Some(item_type.parse::<u64>().unwrap()), children: Default::default() });
            }
        }
    }
    
    inoderoot.print(0);
    let total_used_space = inoderoot.compute_size();
    inoderoot.print(0);
    let mut folders : Vec<InodeRec> = Default::default();
    inoderoot.retrieve_folders_from_size(&mut folders, 100000);
    println!("Total size of directories below threshold is {}", folders.iter().map(|f|f.size.unwrap()).sum::<u64>());
    let disk_size = 70000000;
    let unused_space = disk_size - total_used_space;
    let required_space = 30000000;
    let needed_to_free = required_space - unused_space;
    println!("We need to free {}", needed_to_free);
    let mut folders2 : Vec<InodeRec> = Default::default();
    inoderoot.retrieve_folders_from_size_free(&mut folders2, needed_to_free);
    println!("The smallest of all directories sizes needed to free space is {}", folders2.iter().map(|f|f.size.unwrap()).sorted().collect::<Vec<u64>>().first().unwrap());

}
