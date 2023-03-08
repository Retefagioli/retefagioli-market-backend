package io.exsuslabs.retefagiolimarketbackend.model;

import jakarta.annotation.Nonnull;
import jakarta.annotation.Nullable;
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

    @Nonnull
    String name;
    @Nonnull
    String surname;
    Date dob;
    @Nonnull
    String email;
    String phone;
    String address;

}
