package io.exsuslabs.retefagiolimarketbackend;

import static org.springframework.test.web.servlet.request.MockMvcRequestBuilders.post;

import com.google.gson.Gson;
import io.exsuslabs.retefagiolimarketbackend.controller.UserController;
import io.exsuslabs.retefagiolimarketbackend.request.UserFullInfoRequest;
import io.exsuslabs.retefagiolimarketbackend.service.UserService;
import java.nio.charset.Charset;
import java.util.Optional;
import java.util.UUID;
import org.junit.jupiter.api.Test;
import org.mockito.Mockito;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.autoconfigure.web.servlet.AutoConfigureMockMvc;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.boot.test.context.TestConfiguration;
import org.springframework.boot.test.mock.mockito.MockBean;
import org.springframework.http.MediaType;
import org.springframework.test.web.servlet.MockMvc;
import org.springframework.test.web.servlet.result.MockMvcResultMatchers;
import org.springframework.web.servlet.config.annotation.EnableWebMvc;

@SpringBootTest(classes = {
        TestConfiguration.class,
        UserController.class
})
@AutoConfigureMockMvc
@EnableWebMvc
public class UserMangementTest {

    @Autowired
    MockMvc mockMvc;

    @MockBean
    UserService userService;

    @Test
    void contextLoads() {
    }


    @Test
    public void exampleTest() throws Exception {

        UserFullInfoRequest fullInfoRequest = new UserFullInfoRequest();
        UUID testingUUID = UUID.randomUUID();
        fullInfoRequest.setId(testingUUID);
        fullInfoRequest.setName("test");
        fullInfoRequest.setSurname("last_test");
        fullInfoRequest.setEmail("test@test.com");

        Mockito.when(userService.createUser(fullInfoRequest)).thenReturn(Optional.empty());

        Gson gson = new Gson();

        mockMvc.perform(post("/user")
                        .content(gson.toJson(fullInfoRequest))
                        .accept(MediaType.APPLICATION_JSON)
                        .contentType("application/json")
                        .characterEncoding(Charset.defaultCharset()))
                .andExpect(MockMvcResultMatchers.status().isCreated())
                .andReturn();
    }
}
