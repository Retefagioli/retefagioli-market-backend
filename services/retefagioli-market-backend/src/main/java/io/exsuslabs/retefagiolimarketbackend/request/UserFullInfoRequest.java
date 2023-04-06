package io.exsuslabs.retefagiolimarketbackend.request;


import java.util.Date;
import java.util.UUID;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

@Setter
@Getter
@NoArgsConstructor
public class UserFullInfoRequest {
    UUID id;
    String name;
    String surname;
    Date dob;
    String email;
    String phone;
    String address;
}
