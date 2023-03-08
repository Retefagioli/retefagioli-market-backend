package io.exsuslabs.retefagiolimarketbackend.request;


import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

import java.util.Date;
import java.util.UUID;

@Setter
@Getter
@NoArgsConstructor
public class UserInfoRequest {
    UUID id;
    String name;
    String surname;
    Date dob;
    String email;
    String phone;
    String address;
}
