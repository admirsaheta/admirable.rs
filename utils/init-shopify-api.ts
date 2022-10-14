import chalk from "chalk";
import Shopify from "shopify-typed-node-api";

export const initShopifyApi = () => {
  const { ADMIRABLE_CMS_SHOP, ADMIRABLE_CMS_ACCESS_TOKEN } = process.env;
  if (!ADMIRABLE_CMS_SHOP || !ADMIRABLE_CMS_ACCESS_TOKEN) {
    console.log(
      chalk.red(
        "`ADMIRABLE_CMS_SHOP` or `ADMIRABLE_CMS_ACCESS_TOKEN` are not set."
      )
    );
    throw "Error";
  }

  const api = new Shopify.Clients.Rest(ADMIRABLE_CMS_SHOP, ADMIRABLE_CMS_ACCESS_TOKEN);
  const gql = new Shopify.Clients.Graphql(ADMIRABLE_CMS_SHOP, ADMIRABLE_CMS_ACCESS_TOKEN);

  return { api, gql };
};