import chalk from "chalk";
import inquirer from "inquirer";
import { RestClient } from "shopify-typed-node-api/dist/clients/rest";
import { Theme } from "shopify-typed-node-api/dist/clients/rest/dataTypes";
import { createTheme } from "./create-theme";
import { Config } from "./init-config";

export const initTheme = async (api: RestClient, config: Config): Promise<string | null> => {
  const { ADMIRABLE_CMS_THEME_ID } = process.env;
  try {
    const {
      body: { themes },
    } = await api.get<Theme.Get>({
      path: "themes",
    });

    if (
      !ADMIRABLE_CMS_THEME_ID ||
      !themes.some((theme) => {
        return theme.id === +ADMIRABLE_CMS_THEME_ID;
      })
    ) {
      const { theme_create } = await inquirer.prompt([
        {
          name: "theme_create",
          type: "confirm",
          message:
            "No theme id was provided / or theme id is incorrect - Setup via `ADMIRABLE_CMS_THEME_ID`, do you want to create a new Theme?",
        },
      ]);

      if (theme_create) {
        const { theme_name } = await inquirer.prompt([
          {
            name: "theme_name",
            type: "input",
            message: "Enter a name for your theme",
          },
        ]);

        const { theme_publish } = await inquirer.prompt([
          {
            name: "theme_publish",
            type: "confirm",
            message: "Do you want to publish the theme?",
          },
        ]);

        const theme = await createTheme(api, theme_name, theme_publish, config);

        console.log(chalk.red.bold(`NEW THEME ID: "${theme.id}"`));
        return String(theme.id);
      }

      if (!ADMIRABLE_CMS_THEME_ID) {
        console.log(
          chalk.red(
            "`ADMIRABLE_CMS_SHOP` or `ADMIRABLE_CMS_ACCESS_TOKEN` are incorrect. Please ensure that the variables are setup."
          )
        );
        throw "ERROR";
      }
    }
  } catch (err) {
    if (err.response.code === 401) {
      console.log(
        chalk.red(
          "`ADMIRABLE_CMS_SHOP` or `ADMIRABLE_CMS_ACCESS_TOKEN` are incorrect. Please ensure that the variables are setup."
        )
      );
    }
    console.log(chalk.redBright(err.message));
    throw "ERROR";
  }
  return ADMIRABLE_CMS_THEME_ID;
};