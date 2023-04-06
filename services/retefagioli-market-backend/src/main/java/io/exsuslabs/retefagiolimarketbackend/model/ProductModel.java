package io.exsuslabs.retefagiolimarketbackend.model;

import jakarta.annotation.Nonnull;
import jakarta.persistence.*;
import javax.swing.ImageIcon;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

@NoArgsConstructor
@Getter
@Setter
public class ProductModel {
    

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    Long id;
    
    ImageIcon image;
    Long stockQuantity;
    @Nonnull
    String measure;
    ProductModel aggregates;
    @Nonnull
    CategoryModel category;
    @Nonnull
    SupplierModel supplier;
    @Nonnull
    Float taxablePurchase;
    @Nonnull
    Float taxableSale;
    @Nonnull 
    Float quote;
}
