package io.exsuslabs.retefagiolimarketbackend;

import javax.sql.DataSource;
import org.mockito.Mockito;
import org.springframework.context.annotation.Bean;
import org.springframework.web.servlet.config.annotation.EnableWebMvc;

@org.springframework.boot.test.context.TestConfiguration
@EnableWebMvc
public class TestConfiguration {
    @Bean
    public DataSource dataSource () {
        return Mockito.mock(DataSource.class);
    }
}
