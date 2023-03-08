package io.exsuslabs.retefagiolimarketbackend.repository;

import io.exsuslabs.retefagiolimarketbackend.model.UserModel;
import org.springframework.data.repository.CrudRepository;
import org.springframework.stereotype.Repository;

import java.util.UUID;

@Repository
public interface UserRepository extends CrudRepository<UserModel, UUID> {
}
