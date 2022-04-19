use super::super::user::{User, UserRole};

fn get_test_roles<'a>() -> (UserRole<'a>, UserRole<'a>) {
    let ra: UserRole = UserRole::new("ROLE_A", vec!["NSA/1", "NSA/2", "NSB/1", "NSB/2"]);
    let rb: UserRole = UserRole::new("ROLE_B", vec!["NSC/1", "NSC/2", "NSD/1", "NSD/2"]);
    (ra, rb)
}

#[test]
fn simple_action_match_true() {
    let mut user = User::new("foo".to_string());
    let (ra, rb) = get_test_roles();
    user.roles = vec![ra, rb];
    let has_action = user.has_action("NSA/1");
    assert!(has_action);
}

#[test]
fn simple_action_match_false() {
    let mut user = User::new("foo".to_string());
    let (ra, rb) = get_test_roles();
    user.roles = vec![ra, rb];
    let has_action = user.has_action("NSA/1");
    assert!(has_action);
}

#[test]
fn simple_action_match_true_duplicated_action() {
    let mut user = User::new("foo".to_string());
    let (ra, rb) = get_test_roles();
    user.roles = vec![ra, rb];
    let has_action = user.has_action("INVALID");
    assert!(has_action);
}
