fn main() {
    println!("Hello, world!");

    let mut items: Vec<OrderItems> = vec![];

    let order_item = OrderItems {
        order_id: 1,
        product_id: 1244,
        price: 150.0,
        amount: 1,
    };

    items.push(order_item);

    let mut order = Order {
        id: 1,
        customer_id: 0,
        total: 150.0,
        discount: 0.0,
        description: String::from("It should have a nice taste"),
        status: OrderStatus::Ordered,
        items,
    };

    order.set_order_status(OrderStatus::Waiting);

    println!("Order Status is: {:?}", order.status);

    println!("I'm chancing order status now");

    order.set_order_status(OrderStatus::Delivered);

    println!("Order Status is: {:?}", order.status);
}

#[derive(Debug, PartialEq)]
enum OrderStatus {
    Ordered,
    Waiting,
    OnTheWay,
    Delivered,
    Closed,
}

struct OrderItems {
    order_id: i32,
    product_id: i32,
    amount: i32,
    price: f64,
}

struct Order {
    id: i32,
    customer_id: i32,
    total: f64,
    discount: f64,
    description: String,
    status: OrderStatus,
    items: Vec<OrderItems>,
}

impl Order {
    fn set_order_status(&mut self, status: OrderStatus) {
        self.status = status;
    }

    fn get_order_status(self) -> OrderStatus {
        return self.status;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_status() {
        let mut items: Vec<OrderItems> = vec![];

        let order_item = OrderItems {
            order_id: 1,
            product_id: 1244,
            price: 150.0,
            amount: 1,
        };

        items.push(order_item);

        let mut order = Order {
            id: 1,
            customer_id: 0,
            total: 150.0,
            discount: 0.0,
            description: String::from("It should have a nice taste"),
            status: OrderStatus::Ordered,
            items,
        };

        order.set_order_status(OrderStatus::Delivered);

        assert_eq!(
            order.status,
            OrderStatus::Delivered,
            "Order status different than you expect {:?}",
            order.status
        );
    }
}
