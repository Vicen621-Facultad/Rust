/*use std::collections::HashMap;
use crate::practica3::ej3::Fecha;

struct StreamingRust<'a> {
    users: Vec<User>,
    subscriptions: Vec<Subscription<'a>>
}

struct User {
    id: u32,
    name: String,
    payment_method: PaymentMethod,
}

struct Subscription<'a> {
    subscription_type: SubscriptionType,
    state: SubscriptionState,
    duration_months: u32,
    start_date: Fecha,
    user: &'a User
}

#[derive(Eq, PartialEq)]
enum SubscriptionState {
    Active,
    Inactive,
}

#[derive(Eq, PartialEq, Hash, Clone)]
enum SubscriptionType {
    Basic,
    Clasic,
    Super,
}

#[derive(Eq, PartialEq, Hash, Clone)]
enum PaymentMethod {
    Cash,
    MercadoPago { account_id: String },
    CreditCard { card_number: String, expiration_date: String },
    BankTransfer { bank_account: String },
    Crypto { wallet_address: String },
}

impl<'a> StreamingRust<'a> {
    fn new() -> Self {
        StreamingRust {
            users: Vec::new(),
            subscriptions: Vec::new()
        }
    }
}

trait UserManagement {
    fn create_subscription(&mut self, user_id: u32, name: String, subscription_type: SubscriptionType, duration_months: u32, payment_method: PaymentMethod);
    fn create_user(&mut self, user_id: u32, name: String, payment_method: PaymentMethod) -> &User;
    fn get_user(&self, user_id: u32) -> Option<&User>;
}

impl<'a> UserManagement for StreamingRust<'a> {
    fn create_subscription(&mut self, user_id: u32, name: String, subscription_type: SubscriptionType, duration_months: u32, payment_method: PaymentMethod) {
        if self.get_user(user_id).is_none() {
            self.create_user(user_id, name.clone(), payment_method.clone());
        }

        let user = self.get_user(user_id).unwrap();
        let subscription = Subscription::new(subscription_type, duration_months, &user);
        self.subscriptions.push(subscription);
    }

    fn create_user(&mut self, user_id: u32, name: String, payment_method: PaymentMethod) -> &User {
        let user = User::new(user_id, name, payment_method);
        self.users.push(user);
        self.users.last().unwrap()
    }

    fn get_user(&self, user_id: u32) -> Option<&User> {
        self.users.iter().find(|user| user.id == user_id)   
    }
}

trait Statistics {
    fn most_used_active_payment_method(&self) -> Option<PaymentMethod>;
    fn most_popular_active_subscription(&self) -> Option<SubscriptionType>;
    fn most_used_payment_method(&self) -> Option<PaymentMethod>;
    fn most_popular_subscription(&self) -> Option<SubscriptionType>;
}

impl Statistics for StreamingRust<'_> {
    fn most_used_active_payment_method(&self) -> Option<PaymentMethod> {
        let mut payment_methods = HashMap::new();
        self.subscriptions.iter().filter(|subscription| subscription.is_active()).for_each(|subscription| {
            *payment_methods.entry(subscription.user.payment_method.clone()).or_insert(0) += 1;
        });

        payment_methods.iter().max_by_key(|(_, count)| *count).map(|(subscription_type, _)| subscription_type.clone())
    }

    fn most_popular_active_subscription(&self) -> Option<SubscriptionType> {
        let mut subscriptions = HashMap::new();
        self.subscriptions.iter().filter(|subscription| subscription.is_active()).for_each(|subscription| {
            *subscriptions.entry(subscription.subscription_type.clone()).or_insert(0) += 1;
        });

        subscriptions.iter().max_by_key(|(_, count)| *count).map(|(subscription_type, _)| subscription_type.clone())
    }

    fn most_used_payment_method(&self) -> Option<PaymentMethod> {
        let mut payment_methods = HashMap::new();
        for user in &self.users {
            *payment_methods.entry(user.payment_method.clone()).or_insert(0) += 1;
        }

        payment_methods.iter().max_by_key(|(_, count)| *count).map(|(payment_method, _)| payment_method.clone())
    }

    fn most_popular_subscription(&self) -> Option<SubscriptionType> {
        let mut subscriptions = HashMap::new();
        for subscription in &self.subscriptions {
            *subscriptions.entry(subscription.subscription_type.clone()).or_insert(0) += 1;
        }

        subscriptions.iter().max_by_key(|(_, count)| *count).map(|(subscription_type, _)| subscription_type.clone())
    }
}
trait SubscriptionManagement {
    fn upgrade_subscription(&mut self, user_id: u32);
    fn downgrade_subscription(&mut self, user_id: u32);
    fn cancel_subscription(&mut self, user_id: u32);
    fn get_subscription(&mut self, user_id: u32) -> Option<&mut Subscription>;
}

impl SubscriptionManagement for StreamingRust<'_> {
    fn upgrade_subscription(&mut self, user_id: u32) {
        let subscription: Option<&mut Subscription> = self.get_subscription(user_id);
        if let Some(subscription) = subscription {
            subscription.upgrade();
        }
    }

    fn downgrade_subscription(&mut self, user_id: u32) {
        let subscription = self.get_subscription(user_id);
        if let Some(subscription) = subscription {
            subscription.downgrade();
        }
    }

    fn cancel_subscription(&mut self, user_id: u32) {
        let subscription = self.get_subscription(user_id);
        if let Some(subscription) = subscription {
            subscription.cancel();
        }
    }

    //INCOMPLETE: I don't know how to return a mutable reference to a Subscription
    fn get_subscription(&mut self, _user_id: u32) -> Option<&mut Subscription> {
        todo!("Implementar SubscriptionManagement::get_subscription");
    }
}

impl User {
    fn new(id: u32, name: String, payment_method: PaymentMethod) -> Self {
        User {
            id,
            name,
            payment_method
        }
    }

}

impl<'a> Subscription<'a> {
    fn new(subscription_type: SubscriptionType, duration_months: u32, user: &'a User) -> Self {
        Subscription {
            subscription_type,
            state: SubscriptionState::Active,
            duration_months,
            start_date: Fecha::now(),
            user
        }
    }

    fn upgrade(&mut self) {
        self.subscription_type = match self.subscription_type {
            SubscriptionType::Basic => SubscriptionType::Clasic,
            SubscriptionType::Clasic => SubscriptionType::Super,
            SubscriptionType::Super => SubscriptionType::Super, // No se puede mejorar más
        };
    }

    fn downgrade(&mut self) {
        self.subscription_type = match self.subscription_type {
            SubscriptionType::Super => SubscriptionType::Clasic,
            SubscriptionType::Clasic => SubscriptionType::Basic,
            SubscriptionType::Basic => {
                self.cancel();
                return;
            }
        };
    }

    fn cancel(&mut self) {
        self.state = SubscriptionState::Inactive;
    }

    fn cost(&self) -> f64 {
        self.subscription_type.cost() * self.duration_months as f64
    }

    fn is_active(&self) -> bool {
        self.state == SubscriptionState::Active
    }
}

impl SubscriptionType {
    fn cost(&self) -> f64 {
        match self {
            SubscriptionType::Basic => 10.0,
            SubscriptionType::Clasic => 20.0,
            SubscriptionType::Super => 30.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        
    }
}*/