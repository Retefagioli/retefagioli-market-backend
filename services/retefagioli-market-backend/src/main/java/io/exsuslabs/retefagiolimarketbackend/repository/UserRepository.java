package io.exsuslabs.retefagiolimarketbackend.repository;

import io.exsuslabs.retefagiolimarketbackend.model.UserModel;
import java.util.UUID;
import org.springframework.data.repository.CrudRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface UserRepository extends CrudRepository<UserModel, UUID> {
}
