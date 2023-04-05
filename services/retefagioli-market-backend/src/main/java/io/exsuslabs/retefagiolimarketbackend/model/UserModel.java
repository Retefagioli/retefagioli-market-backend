package io.exsuslabs.retefagiolimarketbackend.model;

import jakarta.annotation.Nonnull;
import jakarta.persistence.*;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

import java.util.Date;
import java.util.UUID;

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
