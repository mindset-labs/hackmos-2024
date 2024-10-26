import "@/styles/globals.css";
import type { AppProps } from "next/app";
import { ChainProvider } from "@cosmos-kit/react";
import { chains, assets } from "chain-registry";
import { wallets } from "@cosmos-kit/keplr";
import "@interchain-ui/react/styles";
import mantraChain from "../config/mantrachaintestnet2/chain";
import mantraAssetList from "../config/mantrachaintestnet2/assetlist";

export default function App({ Component, pageProps }: AppProps) {
  return (
    <ChainProvider
      chains={[...chains, mantraChain]}
      assetLists={[...assets, mantraAssetList]}
      wallets={wallets} // supported wallets
    >
      <Component {...pageProps} />
    </ChainProvider>
  );
}
