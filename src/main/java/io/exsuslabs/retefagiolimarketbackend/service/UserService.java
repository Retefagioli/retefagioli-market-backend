package io.exsuslabs.retefagiolimarketbackend.service;

import io.exsuslabs.retefagiolimarketbackend.request.UserFullInfoRequest;

import java.util.Optional;
import java.util.UUID;

public interface UserService {
    Optional<String> createUser(UserFullInfoRequest userFullInfoRequest);
    Optional<String> updateUser(UserFullInfoRequest userFullInfoRequest);
    Optional<String> getUser(UUID id);
    Optional<String> deleteUser(UUID id);
}
