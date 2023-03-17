package io.exsuslabs.retefagiolimarketbackend.service;

import com.auth0.jwt.interfaces.DecodedJWT;
import io.exsuslabs.retefagiolimarketbackend.model.UserModel;

import java.util.Optional;

public interface JWTService {
    Optional<String> createJWT(UserModel userModel);
    Optional<DecodedJWT> verifyJWT(String token);
}
