#include <stdio.h>
#include <stdlib.h>
#include <regex.h>
#include <string.h>

int main()
{
    FILE* tsp_file;
    char* line;
    size_t len = 0;

    char* reg_pattern_header = "([A-Z_]+)\\s*:\\s*(.+)";
    regex_t reg_header;
    size_t max_groups = 3;
    regmatch_t matched_groups[max_groups];

    if (regcomp(&reg_header, reg_pattern_header, REG_EXTENDED))
    {
        printf("Could not compile regular expression.\n");
        exit(-1);
    }

    tsp_file = fopen("tsplib95/ALL_tsp/a280.tsp", "r");

    if (tsp_file == NULL)
    {
        printf("Error! Could not open file\n");
        exit(-1);
    }

    while (getline(&line, &len, tsp_file) != -1)
    {
        //printf("%s\n", line);

        if (regexec(&reg_header, line, max_groups, matched_groups, 0) == 0)
        {
            char* header[max_groups - 1];

            for (int8_t g = 1; g < max_groups; ++g)
            {
                unsigned int field_len = matched_groups[g].rm_eo - matched_groups[g].rm_so;
                char header_value[field_len];
                strncpy(header_value, line + matched_groups[g].rm_so, field_len);
                header_value[field_len] = '\0';
//                printf("Extracted value[%d]: %s\n", g-1, header_value);
                header[g - 1] = header_value;
                printf("Extracted value[%d]: %s\n", g-1, header[g-1]);
            }

            printf("Header Name: %s - Header Value: %s\n", header[0], header[1]);

//            unsigned int field_len = matched_groups[1].rm_eo - matched_groups[1].rm_so;
//            char header_value[field_len];
//            strncpy(header_value, line + matched_groups[1].rm_so, field_len);
//            header_value[field_len] = '\0';
//
//            printf("Header in --> %s\n", line);
//            printf("Field: %s\n", header_value);
        }
    }

    fclose(tsp_file);
    regfree(&reg_header);

    if (line)
    {
        free(line);
    }

    return 0;
}

void extract_header(regmatch_t* matched_groups, char* line, char* header_value)
{
    unsigned int field_len = matched_groups[1].rm_eo - matched_groups[1].rm_so;
    //char header_value[field_len];
    strncpy(header_value, line + matched_groups[1].rm_so, field_len);
    header_value[field_len] = '\0';
}
