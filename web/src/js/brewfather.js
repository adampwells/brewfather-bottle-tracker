export default {
    async getConditioningBatches() {

        let brewfatherId = localStorage.getItem('brewfatherId');
        let brewfatherKey = localStorage.getItem('brewfatherKey');
        let headers = new Headers();
        headers.set('Authorization', 'Basic ' + window.btoa(brewfatherId + ":" + brewfatherKey));

        const response = await fetch('https://api.brewfather.app/v2/batches?status=Conditioning&limit=50&include=measuredFermenterTopUp,measuredMashPh&order_by=batchNo', {
            method:'GET',
            mode: 'cors',
            redirect: 'follow',
            headers: headers,
        })

        return response.json()

    },

    async updateBatch(batchId, measuredMashPh, measuredFermenterTopUp) {

        let brewfatherId = localStorage.getItem('brewfatherId');
        let brewfatherKey = localStorage.getItem('brewfatherKey');
        let headers = new Headers();
        headers.set('Authorization', 'Basic ' + window.btoa(brewfatherId + ":" + brewfatherKey));

        const response = await fetch(`https://api.brewfather.app/v2/batches/${batchId}?measuredMashPh=${measuredMashPh}&measuredFermenterTopUp=${measuredFermenterTopUp}`, {
            method:'PATCH',
            mode: 'cors',
            redirect: 'follow',
            headers: headers,
        })

        return response.json()

    },

}
