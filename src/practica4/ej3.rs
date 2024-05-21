use crate::practica3::ej3::Fecha;

struct StreamingRust {
    users: Vec<User>
}

struct User {
    id: u32,
    name: String,
    subscription: Option<Subscription>,
    payment_method: Option<PaymentMethod>,
}

struct Subscription {
    subscription_type: SubscriptionType,
    cost: f64,
    duration_months: u32,
    start_date: Fecha,
}

enum SubscriptionType {
    Basic,
    Clasic,
    Super,
}

enum PaymentMethod {
    Cash,
    MercadoPago { account_id: String },
    CreditCard { card_number: String, expiration_date: String },
    BankTransfer { bank_account: String },
    Crypto { wallet_address: String },
}

trait UserManagement {
    fn create_user(&mut self, user_id: u32, name: String, subscription: Subscription, payment_method: PaymentMethod) -> &User;
}

impl UserManagement for StreamingRust {
    fn create_user(&mut self, id: u32, name: String, subscription: Subscription, payment_method: PaymentMethod) -> &User {
        let user = User {
            id,
            name,
            subscription: Some(subscription),
            payment_method: Some(payment_method)
        };
        self.users.push(user);
        self.users.last().unwrap()
    }
}

impl StreamingRust {
    
}

trait SubscriptionManagement {
    fn upgrade_subscription(&mut self);
    fn downgrade_subscription(&mut self);
    fn cancel_subscription(&mut self);
}

impl SubscriptionManagement for User {
    fn upgrade_subscription(&mut self) {
        if let Some(subscription) = &mut self.subscription {
            subscription.subscription_type = match subscription.subscription_type {
                SubscriptionType::Basic => SubscriptionType::Clasic,
                SubscriptionType::Clasic => SubscriptionType::Super,
                SubscriptionType::Super => SubscriptionType::Super, // No se puede mejorar más
            };
        }
    }

    fn downgrade_subscription(&mut self) {
        if let Some(subscription) = &mut self.subscription {
            subscription.subscription_type = match subscription.subscription_type {
                SubscriptionType::Super => SubscriptionType::Clasic,
                SubscriptionType::Clasic => SubscriptionType::Basic,
                SubscriptionType::Basic => {
                    self.cancel_subscription();
                    return;
                }
            };
        }
    }

    fn cancel_subscription(&mut self) {
        self.subscription = None;
    }
}