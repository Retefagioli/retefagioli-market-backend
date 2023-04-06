package io.exsuslabs.retefagiolimarketbackend.service;

import io.exsuslabs.retefagiolimarketbackend.model.UserModel;
import java.util.Optional;

public interface JWTService {
    Optional<String> createJWT(UserModel userModel);
}
