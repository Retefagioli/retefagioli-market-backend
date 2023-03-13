package io.exsuslabs.retefagiolimarketbackend.model;

import jakarta.annotation.Nonnull;
import jakarta.persistence.*;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

@NoArgsConstructor
@Getter
@Setter
public class SupplierModel {

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    Long id;

    String description;
    @Nonnull
    String IVA;
    @Nonnull
    String companyOffice;

    String address;

    // TUTTI GLI ATTRIBUTI CHE SI VOGLIONO AGGIUNGERE ...

}
