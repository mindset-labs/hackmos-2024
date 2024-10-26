"use-client";

import CreateTrustForm from "@/components/CreateTrust";
import Navbar from "@/components/Navbar";

export default function Create() {
  return (
    <div className="bg-white">
      <Navbar />
      <div className="mt-14">
        <CreateTrustForm />
      </div>
    </div>
  );
}
