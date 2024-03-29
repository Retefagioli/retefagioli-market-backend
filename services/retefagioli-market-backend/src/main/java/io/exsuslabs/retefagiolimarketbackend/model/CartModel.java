package io.exsuslabs.retefagiolimarketbackend.model;

import jakarta.annotation.Nonnull;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import java.util.Date;
import java.util.TreeMap;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

@NoArgsConstructor
@Getter
@Setter
public class CartModel {

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    Long id;

    @Nonnull
    UserModel user;
    @Nonnull
    TreeMap<ProductModel, Integer> products;
    @Nonnull
    boolean isConcluded;
    Date openingDate;

}
