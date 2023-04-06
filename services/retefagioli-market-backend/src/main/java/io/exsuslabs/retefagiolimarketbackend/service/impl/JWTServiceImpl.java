package io.exsuslabs.retefagiolimarketbackend.service.impl;

import com.auth0.jwt.JWT;
import com.auth0.jwt.JWTVerifier;
import com.auth0.jwt.algorithms.Algorithm;
import com.auth0.jwt.exceptions.JWTCreationException;
import com.auth0.jwt.exceptions.JWTVerificationException;
import com.auth0.jwt.interfaces.DecodedJWT;
import io.exsuslabs.retefagiolimarketbackend.model.UserModel;
import io.exsuslabs.retefagiolimarketbackend.service.JWTService;
import java.time.Instant;
import java.util.Optional;

public class JWTServiceImpl implements JWTService {
    private static final String SECRET_KEY = "";
    private static final String ISSUER = "EXSUS-LABS-001";
    private static final Algorithm ALGO = Algorithm.HMAC256(SECRET_KEY);
    public Optional<String> createJWT(UserModel userModel) {
        try {
            return Optional.of(JWT
                    .create()
                    .withSubject(userModel.getName())
                    .withIssuer(ISSUER)
                    .withExpiresAt(Instant.now().plusSeconds(300))
                    .withIssuedAt(Instant.now())
                    .withClaim("id", userModel.getId().toString())
                    .withClaim("email", userModel.getEmail())
                    .sign(ALGO)
            );
        } catch (JWTCreationException e) {
            return Optional.empty();
        }
    }

    public Optional<DecodedJWT> verifyJWT(String token) {
        try {
            JWTVerifier verifier = JWT.require(ALGO)
                    .withIssuer(ISSUER)
                    .withClaimPresence("id")
                    .withClaimPresence("email")
                    .build();
            return Optional.of(verifier.verify(token));
        } catch (JWTVerificationException e) {
            return Optional.empty();
        }
    }
}
