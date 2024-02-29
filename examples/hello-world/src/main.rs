use brief::mail::MessageBuilder;

fn main() {
    let message = MessageBuilder::new()
        .from("name <user@domain.com".parse().unwrap())
        .to("nametwo <usertwo@domaintwo.com>".parse().unwrap())
        .build();

    // TODO: send
}
