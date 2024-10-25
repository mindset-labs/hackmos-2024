import "@/styles/globals.css";
import type { AppProps } from "next/app";
import { ChainProvider } from "@cosmos-kit/react";
import { chains, assets } from "chain-registry";
import { wallets } from "@cosmos-kit/keplr";
import "@interchain-ui/react/styles";

export default function App({ Component, pageProps }: AppProps) {
  return (
    <ChainProvider
      chains={chains} // supported chains
      assetLists={assets} // supported asset lists
      wallets={wallets} // supported wallets
    >
      <Component {...pageProps} />
    </ChainProvider>
  );
}
