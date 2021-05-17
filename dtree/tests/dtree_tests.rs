/// Directory Tree Simulator tests
#[cfg(test)]
mod tests {
    use dtree::*;

    #[test]
    fn test_new_dir_entry_correct_name() {
        let dir = DEnt::new("test");
        assert_eq!(dir.is_ok(), true);
    }

    #[test]
    fn test_new_dir_entry_single_slash_in_name() {
        let dir = DEnt::new("/test");
        assert_eq!(dir.is_ok(), false);
    }

    #[test]
    fn test_new_dir_entry_multiple_slash_in_name() {
        let dir = DEnt::new("t/e/s/t");
        assert_eq!(dir.is_ok(), false);
    }

    #[test]
    fn test_mkdir_result() {
        let mut dt = DTree::new();
        let result = dt.mkdir("test");
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_mkdir_error_slash_in_name() {
        let mut dt = DTree::new();
        let result = dt.mkdir("tes/t");
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_mkdir_error_dir_already_exists() {
        let mut dt = DTree::new();
        dt.mkdir("test").unwrap();
        let result = dt.mkdir("test");
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_with_subdir_empty_tree() {
        let dt = DTree::new();
        let result = dt.with_subdir(&["not a directory"], |_| {});
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_with_subdir_empty_path() {
        let dt = DTree::new();
        let result = dt.with_subdir(&[], |_| {});
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_with_subdir_single_child() {
        let mut dt = DTree::new();
        dt.mkdir("a").unwrap();
        let result = dt.with_subdir(&["a"], |_| {});
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_with_subdir_multiple_children() {
        let mut dt = DTree::new();
        dt.mkdir("a").unwrap();
        dt.mkdir("b").unwrap();
        dt.mkdir("c").unwrap();
        let result = dt.with_subdir(&["a"], |_| {});
        assert_eq!(result.is_ok(), true);

        let result = dt.with_subdir(&["b"], |_| {});
        assert_eq!(result.is_ok(), true);

        let result = dt.with_subdir(&["c"], |_| {});
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_with_subdir_mut_empty_tree() {
        let mut dt = DTree::new();
        let result = dt.with_subdir_mut(&["not a directory"], |_| {});
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_with_subdir_mut_multiple_descendants() {
        let mut dt = DTree::new();
        dt.mkdir("a").unwrap();
        dt.with_subdir_mut(&["a"], |dt| dt.mkdir("b").unwrap())
            .unwrap();
        dt.with_subdir_mut(&["a", "b"], |dt| dt.mkdir("c").unwrap())
            .unwrap();
        assert_eq!(&dt.paths(), &["/a/b/c/"]);
    }

    #[test]
    fn test_paths_empty_tree() {
        let dt = DTree::new();
        assert_eq!(&dt.paths(), &["/"]);
    }

    #[test]
    fn test_paths_multiple_children() {
        let mut dt = DTree::new();
        dt.mkdir("a").unwrap();
        dt.mkdir("b").unwrap();
        dt.mkdir("c").unwrap();
        assert_eq!(&dt.paths(), &["/a/", "/b/", "/c/"]);
    }

    #[test]
    fn test_paths_multiple_descendants() {
        let mut dt = DTree::new();
        dt.mkdir("a").unwrap();
        dt.with_subdir_mut(&["a"], |dt| dt.mkdir("b").unwrap())
            .unwrap();
        dt.with_subdir_mut(&["a"], |dt| dt.mkdir("c").unwrap())
            .unwrap();
        assert_eq!(&dt.paths(), &["/a/b/", "/a/c/"]);
    }

    #[test]
    fn test_chdir_invalid_child() {
        let mut os = OsState::new();
        let result = os.chdir(&["not a directory"]);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_chdir_empty_path_goes_to_root() {
        let mut os = OsState::new();
        os.mkdir("a").unwrap();
        os.chdir(&["a"]).unwrap();
        os.mkdir("b").unwrap();
        os.chdir(&["b"]).unwrap();
        os.mkdir("c").unwrap();
        let result = os.chdir(&[]);
        assert_eq!(result.is_ok(), true);
        assert_eq!(&os.paths().unwrap(), &["/a/b/c/"]);
    }
}
