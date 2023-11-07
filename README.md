# Lend Protocol  

## Overview  



## How  

- User mints bond using whatever collateral  
- collateral swapped to debt token using market (requires initial liquidity, so dedicated endpoint for authorized users / liquidity providers / MM)   
(implicates hard cap on borrowable, keep it simple but maybe could 2 step: allow minting once enough liquidity?)  
(ye tbh, initial liquidity, then can mint at mid price? sounds pretty good to me, allows for much more potential)    
- then can lend debt token to other user  
- what about exercising of the obligation? => take from pool of locked assets?   
- 

ah ouais mince on avait pas dit qu'on faisait le swap sur le marché pour le debt token?
La je pensais bah mint at mid price monkaHmm
ouais nan le mint semble plus approprié, all good  

juste qu'il y a le MM qui va apporter la liquidité de base sur le marché donc 


comment gérer le initial liquidity provisioning?  


ticker format: YYYY_MMM_AA_D_TOKEN   
2023_DEC_01_Y_USDC   


NEED TO ADD possibility of reimbursing debt with base token if after expiry 
needed since may be mismatch between debt available and base token. Plus users who forgot positions in debt would be locking others out of their liquidity forever  


Collateral ratio <= 1.0, debt / collateral 


## Behaviour  

### Creating a new denom  

1- Send admin message create new denom
2- Contract calls TokenFactory module (with a submessage?) 
3- Register currency? maybe no need for submessage 
"The tokenfactory module allows any account to create a new token with the name factory/{creator address}/{subdenom}."
so no need 


## References  

Hackaton ideas page: https://dorahacks.io/hackathon/illuminate/hackathon-ideas#3
Notional.finance: https://notional.finance/  

