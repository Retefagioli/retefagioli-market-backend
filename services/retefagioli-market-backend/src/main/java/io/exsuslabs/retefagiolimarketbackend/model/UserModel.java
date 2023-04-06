package io.exsuslabs.retefagiolimarketbackend.model;

import jakarta.persistence.*;
import java.util.Date;
import java.util.UUID;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

@Entity
@NoArgsConstructor
@Getter
@Setter
@Table(name = "User", schema = "public")
public class UserModel {
    @Id
    UUID id;

    @Column(nullable = false)
    String name;
    @Column(nullable = false)
    String surname;
    Date dob;
    @Column(nullable = false)
    String email;
    String phone;
    String address;

}
