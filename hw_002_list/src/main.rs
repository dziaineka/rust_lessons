#[derive(Debug, PartialEq)]
pub struct Telemetry {
    pub point: Point,
    pub time_stamp: u64,
}

impl Telemetry {
    pub fn new(point: Point, time_stamp: u64) -> Self {
        Self { point, time_stamp }
    }
}

#[derive(Debug, PartialEq)]
pub struct Point {
    pub latitude: f64,
    pub longitude: f64,
}

impl Point {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ListNode {
    pub val: Telemetry,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: Telemetry) -> Self {
        ListNode { next: None, val }
    }

    pub fn with_next(self, next: ListNode) -> Self {
        let next = Some(Box::new(next));
        Self { next, ..self }
    }

    // method should return part of path between two time points
    pub fn get_part(self, from: u64, to: u64) -> Option<ListNode> {
        let mut current_node: &Option<Box<ListNode>> = &Some(Box::new(self));
        let mut start_node: Option<ListNode>;

        (start_node, current_node) = Self::get_start_node(current_node, from, to);

        while let Some(node) = current_node {
            if (node.val.time_stamp >= from) & (node.val.time_stamp <= to) {
                start_node = Some(Self::add_node(
                    start_node,
                    ListNode::new(Telemetry::new(
                        Point::new(node.val.point.latitude, node.val.point.longitude),
                        node.val.time_stamp,
                    )),
                ))
            }
            current_node = &node.next;
        }

        println!("{:#?}", &start_node);
        start_node
    }

    fn get_start_node(
        mut current_node: &Option<Box<ListNode>>,
        from: u64,
        to: u64,
    ) -> (Option<ListNode>, &Option<Box<ListNode>>) {
        let mut start_node: Option<ListNode> = None;

        while start_node.is_none() {
            if let Some(node) = current_node {
                if (node.val.time_stamp >= from) & (node.val.time_stamp <= to) {
                    start_node = Some(ListNode::new(Telemetry {
                        point: Point {
                            latitude: node.val.point.latitude,
                            longitude: node.val.point.longitude,
                        },
                        time_stamp: node.val.time_stamp,
                    }));
                }

                current_node = &node.next
            } else {
                return (None, current_node);
            }
        }

        (start_node, current_node)
    }

    fn add_node(node: Option<ListNode>, mut leaf: ListNode) -> ListNode {
        let return_node: ListNode;

        if let Some(current_node) = node {
            return_node = ListNode::new(Telemetry::new(
                Point::new(
                    current_node.val.point.latitude,
                    current_node.val.point.longitude,
                ),
                current_node.val.time_stamp,
            ));

            leaf = if let Some(next_node) = current_node.next {
                Self::add_node(Some(*next_node), leaf)
            } else {
                leaf
            };

            return_node.with_next(leaf)
        } else {
            leaf
        }
    }
}

fn main() {
    println!("Rust is fun, they said.");
}

#[cfg(test)]
mod test {
    use super::*;

    fn stub() -> ListNode {
        let first = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 100));
        let second = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 200));
        let third = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 300));
        let fourth = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 400));
        let fifth = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 500));
        let sixth = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 600));
        let seventh = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 700));
        let eighth = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 800));
        let ninth = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 900));
        let tenth = ListNode::new(Telemetry::new(Point::new(0f64, 0f64), 1000));

        let second = second.with_next(first);
        let third = third.with_next(second);
        let fourth = fourth.with_next(third);
        let fifth = fifth.with_next(fourth);
        let sixth = sixth.with_next(fifth);
        let seventh = seventh.with_next(sixth);
        let eighth = eighth.with_next(seventh);
        let ninth = ninth.with_next(eighth);

        tenth.with_next(ninth)
    }

    #[test]
    fn not_found() {
        let list = stub();
        let sublist = list.get_part(50, 70);
        assert!(sublist.is_none());
    }

    #[test]
    fn exact_all() {
        let list = stub();
        let sublist = list.get_part(100, 1000);
        assert!(sublist.is_some());

        let sublist = sublist.unwrap();
        assert_eq!(sublist.val.time_stamp, 1000);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 900);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 800);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 700);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 600);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 500);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 400);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 300);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 200);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 100);
    }

    #[test]
    fn gte_lte_all() {
        let list = stub();
        let sublist = list.get_part(0, 1500);
        assert!(sublist.is_some());

        let sublist = sublist.unwrap();
        assert_eq!(sublist.val.time_stamp, 1000);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 900);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 800);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 700);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 600);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 500);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 400);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 300);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 200);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 100);
    }

    #[test]
    fn exact_sublist() {
        let list = stub();
        let sublist = list.get_part(400, 700);
        assert!(sublist.is_some());

        let sublist = sublist.unwrap();
        assert_eq!(sublist.val.time_stamp, 700);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 600);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 500);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 400);
        assert!(sublist.next.is_none());
    }

    #[test]
    fn gte_lte_sublist() {
        let list = stub();
        let sublist = list.get_part(350, 750);
        assert!(sublist.is_some());

        let sublist = sublist.unwrap();
        assert_eq!(sublist.val.time_stamp, 700);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 600);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 500);

        let sublist = sublist.next.unwrap();
        assert_eq!(sublist.val.time_stamp, 400);
        assert!(sublist.next.is_none());
    }
}
